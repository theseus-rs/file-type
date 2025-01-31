use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27826417: FileFormat = FileFormat {
    id: 27_826_417,
    puid: "wikidata/27826417",
    name: "ActionScript file format",
    extensions: &["as", "as", "as", "as"],
    media_types: &[
        "application/ecmascript",
        "application/x-actionscript",
        "text/actionscript",
        "text/x-actionscript",
    ],
    internal_signatures: &[],
    related_formats: &[],
};
