use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27826417: FileFormat = FileFormat {
    id: 27_826_417,
    source_type: SourceType::Wikidata,
    name: "ActionScript file format",
    extensions: &["as"],
    media_types: &[
        "application/ecmascript",
        "application/x-actionscript",
        "text/actionscript",
        "text/x-actionscript",
    ],
    signatures: &[],
    related_formats: &[],
};
