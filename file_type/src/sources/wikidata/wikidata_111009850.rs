use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111009850: FileFormat = FileFormat {
    id: 111_009_850,
    source_type: SourceType::Wikidata,
    name: "PrintMaster T-Shirt File format",
    extensions: &["tsh"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
