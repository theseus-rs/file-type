use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_51093528: FileFormat = FileFormat {
    id: 51_093_528,
    source_type: SourceType::Wikidata,
    name: "CorelDraw Pattern",
    extensions: &["pat"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
