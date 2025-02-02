use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27979383: FileFormat = FileFormat {
    id: 27_979_383,
    source_type: SourceType::Wikidata,
    name: "Heroglyph Project Format",
    extensions: &["hprj"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
