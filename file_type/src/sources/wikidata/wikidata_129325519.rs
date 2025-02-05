use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_129325519: FileFormat = FileFormat {
    id: 129_325_519,
    source_type: SourceType::Wikidata,
    name: "FunC file format",
    extensions: &["fc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
