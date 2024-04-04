use proc_macro::{Group, Ident, TokenStream, TokenTree};

fn replace_ident(ident: Ident) -> Option<TokenTree> {
    let ident_str = ident.to_string();

    let new_str = match ident_str.as_str() {
        "Xato" => "Err",
        "Yaxshi" => "Ok",
        "Matn" => "String",
        "Lug`at" => "HashMap",
        "Odatiy" => "Default",
        "Xatolik" => "Error",
        "Tanlov" => "Option",
        "Bir" => "Some",
        "Hech" => "None",
        "Natija" => "Result",
        "Shu" => "Self",
        "chiqar" => "println",
        "to`xta" => "break",
        "asinxron" => "async",
        "kut" => "await",
        "takror" => "loop",
        "ko`chir" => "move",
        "quti" => "crate",
        "yetib_bormaydi" => "unreachable_code",
        "day" => "as",
        "konstanta" => "const",
        "hislat" => "trait",
        "xavfli" => "unsafe",
        "da" => "in",
        "dan" => "from",
        "dinamik" => "dyn",
        "och" => "unwrap",
        "odatiy" => "default",
        "ko`chirma" => "as_ref",
        "kch" => "io",
        "kengaytma" => "extern",
        "yo`q" => "false",
        "funksiya" => "fn",
        "ushbu" => "super",
        "kirit" => "insert",
        "ol" => "get",
        "mumkin" => "allow",
        "panika" | "chiq" | "vahima" => "panic",
        "modul" => "mod",
        "o`zgaruvchan" => "mut",
        "yangi" => "new",
        "qayerda" => "where",
        "qachonki" => "for",
        "ol_yoki_kirit" => "get_or_insert_with",
        "asosiy" => "main",
        "ochiq" => "pub",
        "qaytar" => "return",
        "to`ldir" => "impl",
        "manzil" => "ref",
        "solishtir" => "match",
        "agar" => "if",
        "unda" => "else",
        "shu" => "self",
        "joy" => "let",
        "statik" => "static",
        "shakl" => "struct",
        "kutma" => "expect",
        "toki" => "while",
        "yukal" => "use",
        "ga" => "into",
        "ha" => "true",
        "tur" => "enum",
        "Guruh" => "Group",
        "Identifikator" => "Ident",
        "TokenOqim" => "TokenStream",
        "TokenDaraxt" => "TokenTree",
        "matnga" => "to_string",
        "statik_matnga" => "as_str",
        "radius" => "span",
        "Vektor" => "Vec",
        "oqim" => "stream",
        "qo`sh" => "push",
        "kengay" => "extend",
        "delimeter" => "delimiter",
        "Punktuatsiya" => "Punct",
        "Literal" => "Literal",
        "protsedural_makro" => "proc_macro",

        // Extra keywords for lexing
        "cha" => None?,

        _ => &ident_str,
    };

    let new_ident = Ident::new(new_str, ident.span());
    Some(TokenTree::Ident(new_ident))
}

fn replace_tree(tok: TokenTree, out: &mut Vec<TokenTree>) {
    match tok {
        TokenTree::Group(group) => {
            let mut group_elem = Vec::new();
            replace_stream(group.stream(), &mut group_elem);
            let mut new_stream = TokenStream::new();
            new_stream.extend(group_elem);
            out.push(TokenTree::Group(Group::new(group.delimiter(), new_stream)));
        }
        TokenTree::Ident(ident) => {
            if let Some(ident) = replace_ident(ident) {
                out.push(ident);
            }
        }
        TokenTree::Punct(..) | TokenTree::Literal(..) => {
            out.push(tok);
        }
    }
}

fn replace_stream(ts: TokenStream, out: &mut Vec<TokenTree>) {
    for tok in ts {
        replace_tree(tok, out)
    }
}

#[proc_macro]
pub fn zang(item: TokenStream) -> TokenStream {
    let mut returned = Vec::new();
    replace_stream(item, &mut returned);
    let mut out = TokenStream::new();
    out.extend(returned);
    out
}
