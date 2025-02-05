use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_112661167: FileFormat = FileFormat {
    id: 112_661_167,
    source_type: SourceType::Wikidata,
    name: "Motion Analysis HTR File",
    extensions: &["htr"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
