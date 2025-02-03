use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_123194261: FileFormat = FileFormat {
    id: 123_194_261,
    source_type: SourceType::Wikidata,
    name: "Comodo Backup File",
    extensions: &["cbu"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
