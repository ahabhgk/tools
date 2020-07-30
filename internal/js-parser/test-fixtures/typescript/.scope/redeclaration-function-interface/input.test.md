# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test packages/@romejs/js-parser/index.test.ts --update-snapshots` to update.

## `typescript > scope > redeclaration-function-interface`

### `ast`

```javascript
JSRoot {
	comments: Array []
	corrupt: false
	diagnostics: Array []
	directives: Array []
	filename: "input.ts"
	hasHoistedVars: false
	interpreter: undefined
	mtime: undefined
	sourceType: "module"
	syntax: Array ["ts"]
	loc: Object {
		filename: "input.ts"
		end: Object {
			column: 14
			index: 30
			line: 2
		}
		start: Object {
			column: 0
			index: 0
			line: 1
		}
	}
	body: Array [
		JSFunctionDeclaration {
			id: JSBindingIdentifier {
				name: "A"
				loc: Object {
					filename: "input.ts"
					identifierName: "A"
					end: Object {
						column: 10
						index: 10
						line: 1
					}
					start: Object {
						column: 9
						index: 9
						line: 1
					}
				}
			}
			loc: Object {
				filename: "input.ts"
				end: Object {
					column: 15
					index: 15
					line: 1
				}
				start: Object {
					column: 0
					index: 0
					line: 1
				}
			}
			body: JSBlockStatement {
				body: Array []
				directives: Array []
				loc: Object {
					filename: "input.ts"
					end: Object {
						column: 15
						index: 15
						line: 1
					}
					start: Object {
						column: 13
						index: 13
						line: 1
					}
				}
			}
			head: JSFunctionHead {
				async: false
				generator: false
				hasHoistedVars: false
				params: Array []
				rest: undefined
				returnType: undefined
				thisType: undefined
				typeParameters: undefined
				loc: Object {
					filename: "input.ts"
					end: Object {
						column: 12
						index: 12
						line: 1
					}
					start: Object {
						column: 10
						index: 10
						line: 1
					}
				}
			}
		}
		TSInterfaceDeclaration {
			id: JSBindingIdentifier {
				name: "A"
				loc: Object {
					filename: "input.ts"
					identifierName: "A"
					end: Object {
						column: 11
						index: 27
						line: 2
					}
					start: Object {
						column: 10
						index: 26
						line: 2
					}
				}
			}
			extends: undefined
			typeParameters: undefined
			loc: Object {
				filename: "input.ts"
				end: Object {
					column: 14
					index: 30
					line: 2
				}
				start: Object {
					column: 0
					index: 16
					line: 2
				}
			}
			body: TSInterfaceBody {
				body: Array []
				loc: Object {
					filename: "input.ts"
					end: Object {
						column: 14
						index: 30
						line: 2
					}
					start: Object {
						column: 12
						index: 28
						line: 2
					}
				}
			}
		}
	]
}
```

### `diagnostics`

```
✔ No known problems!

```