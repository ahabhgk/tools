use rome_analyze::AnalysisFilter;
use rome_diagnostics::Diagnostic;
use rome_formatter::Printed;
use rome_fs::RomePath;
use rome_js_syntax::{TextRange, TextSize};
use std::ffi::OsStr;

use crate::{
    settings::SettingsHandle,
    workspace::{
        server::AnyParse, FixFileResult, GetSyntaxTreeResult, PullActionsResult, RenameResult,
    },
    RomeError, Rules,
};

use self::{javascript::JsFileHandler, json::JsonFileHandler, unknown::UnknownFileHandler};

mod javascript;
mod json;
mod unknown;

use crate::workspace::FixFileMode;
pub use javascript::JsFormatSettings;

/// Supported languages by Rome
#[derive(Debug, PartialEq)]
pub(crate) enum Language {
    /// JavaScript, TypeScript, JSX, TSX
    JavaScript,
    /// JSON
    Json,
    /// Any language that is not supported
    Unknown,
}

impl From<&str> for Language {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "js" | "ts" | "jsx" | "tsx" | "mjs" | "cjs" | "cts" | "mts" => Language::JavaScript,
            "json" => Language::Json,
            _ => Language::Unknown,
        }
    }
}

impl From<&OsStr> for Language {
    fn from(s: &OsStr) -> Self {
        match s.to_str().unwrap() {
            "js" | "ts" | "jsx" | "tsx" | "mjs" | "cjs" | "cts" | "mts" => Language::JavaScript,
            "json" => Language::Json,
            _ => Language::Unknown,
        }
    }
}

// TODO: The Css variant is unused at the moment
#[allow(dead_code)]
pub(crate) enum Mime {
    Javascript,
    Json,
    Css,
    Text,
}

impl std::fmt::Display for Mime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Mime::Css => write!(f, "text/css"),
            Mime::Json => write!(f, "application/json"),
            Mime::Javascript => write!(f, "application/javascript"),
            Mime::Text => write!(f, "text/plain"),
        }
    }
}

pub struct FixAllParams<'a> {
    pub(crate) rome_path: &'a RomePath,
    pub(crate) parse: AnyParse,
    pub(crate) rules: Option<&'a Rules>,
    pub(crate) fix_file_mode: FixFileMode,
}

#[derive(Default)]
/// The list of capabilities that are available for a language
pub(crate) struct Capabilities {
    pub(crate) parser: ParserCapabilities,
    pub(crate) debug: DebugCapabilities,
    pub(crate) analyzer: AnalyzerCapabilities,
    pub(crate) formatter: FormatterCapabilities,
}

type Parse = fn(&RomePath, &str) -> AnyParse;

#[derive(Default)]
pub(crate) struct ParserCapabilities {
    /// Parse a file
    pub(crate) parse: Option<Parse>,
}

type DebugSyntaxTree = fn(&RomePath, AnyParse) -> GetSyntaxTreeResult;
type DebugControlFlow = fn(&RomePath, AnyParse, TextSize) -> String;
type DebugFormatterIR = fn(&RomePath, AnyParse, SettingsHandle) -> Result<String, RomeError>;

#[derive(Default)]
pub(crate) struct DebugCapabilities {
    /// Prints the syntax tree
    pub(crate) debug_syntax_tree: Option<DebugSyntaxTree>,
    /// Prints the control flow graph
    pub(crate) debug_control_flow: Option<DebugControlFlow>,
    /// Prints the formatter IR
    pub(crate) debug_formatter_ir: Option<DebugFormatterIR>,
}

type Lint = fn(&RomePath, AnyParse, AnalysisFilter, Option<&Rules>) -> Vec<Diagnostic>;
type CodeActions = fn(&RomePath, AnyParse, TextRange, Option<&Rules>) -> PullActionsResult;
type FixAll = fn(FixAllParams) -> Result<FixFileResult, RomeError>;
type Rename = fn(&RomePath, AnyParse, TextSize, String) -> Result<RenameResult, RomeError>;

#[derive(Default)]
pub(crate) struct AnalyzerCapabilities {
    /// It lints a file
    pub(crate) lint: Option<Lint>,
    /// It extracts code actions for a file
    pub(crate) code_actions: Option<CodeActions>,
    /// Applies fixes to a file
    pub(crate) fix_all: Option<FixAll>,
    /// It renames a binding inside a file
    pub(crate) rename: Option<Rename>,
}

type Format = fn(&RomePath, AnyParse, SettingsHandle) -> Result<Printed, RomeError>;
type FormatRange = fn(&RomePath, AnyParse, SettingsHandle, TextRange) -> Result<Printed, RomeError>;
type FormatOnType = fn(&RomePath, AnyParse, SettingsHandle, TextSize) -> Result<Printed, RomeError>;

#[derive(Default)]
pub(crate) struct FormatterCapabilities {
    /// It formats a file
    pub(crate) format: Option<Format>,
    /// It formats a portion of text of a file
    pub(crate) format_range: Option<FormatRange>,
    /// It formats a file while typing
    pub(crate) format_on_type: Option<FormatOnType>,
}

/// Main trait to use to add a new language to Rome
pub(crate) trait ExtensionHandler {
    /// The language of the file. It can be a super language.
    /// For example, a ".js" file can have [Language::Ts]
    fn language(&self) -> Language;

    /// MIME types used to identify a certain language
    fn mime(&self) -> Mime;

    /// A file that can support tabs inside its content
    fn may_use_tabs(&self) -> bool {
        true
    }

    /// Capabilities that can applied to a file
    fn capabilities(&self) -> Capabilities {
        Capabilities::default()
    }

    /// How a file should be treated. Usually an asset doesn't posses a parser.
    ///
    /// An image should me parked as asset.
    fn is_asset(&self) -> bool {
        false
    }
}

/// Features available for each language
pub(crate) struct Features {
    js: JsFileHandler,
    json: JsonFileHandler,
    unknown: UnknownFileHandler,
}

impl Features {
    pub(crate) fn new() -> Self {
        Features {
            js: JsFileHandler {},
            json: JsonFileHandler {},
            unknown: UnknownFileHandler::default(),
        }
    }

    /// Return a [Language] from a string
    fn get_language(rome_path: &RomePath) -> Language {
        match rome_path.extension() {
            Some(file_extension) => file_extension.into(),
            None => Language::Unknown,
        }
    }

    /// Returns the [Capabilities] associated with a [RomePath]
    pub(crate) fn get_capabilities(&self, rome_path: &RomePath) -> Capabilities {
        match Self::get_language(rome_path) {
            Language::JavaScript => self.js.capabilities(),
            Language::Json => self.json.capabilities(),
            Language::Unknown => self.unknown.capabilities(),
        }
    }
}
