use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_13012348: FileFormat = FileFormat {
    id: 13_012_348,
    source_type: SourceType::Wikidata,
    name: "Adobe Flash project",
    extensions: &["fla"],
    media_types: &["application/vnd.adobe.fla"],
    signatures: &[],
    related_formats: &[],
};
