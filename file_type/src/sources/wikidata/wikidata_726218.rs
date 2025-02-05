use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_726218: FileFormat = FileFormat {
    id: 726_218,
    source_type: SourceType::Wikidata,
    name: "XML User Interface Language",
    extensions: &["xul"],
    media_types: &["application/vnd.mozilla.xul+xml"],
    signatures: &[],
    related_formats: &[],
};
