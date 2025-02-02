use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_129325519: FileFormat = FileFormat {
    id: 129_325_519,
    source_type: SourceType::Wikidata,
    name: "FunC file format",
    extensions: &["fc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
