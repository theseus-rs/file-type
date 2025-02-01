use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_126951804: FileFormat = FileFormat {
    id: 126_951_804,
    puid: "wikidata/126951804",
    name: "Prolog source code file",
    extensions: &["ecl", "pl", "pro", "prolog"],
    media_types: &[
        "text/x-prolog",
        "text/x-prolog",
        "text/x-prolog",
        "text/x-prolog",
    ],
    internal_signatures: &[],
    related_formats: &[],
};
