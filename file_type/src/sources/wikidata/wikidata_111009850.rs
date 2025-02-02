use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111009850: FileFormat = FileFormat {
    id: 111_009_850,
    source_type: SourceType::Wikidata,
    name: "PrintMaster T-Shirt File format",
    extensions: &["tsh"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
