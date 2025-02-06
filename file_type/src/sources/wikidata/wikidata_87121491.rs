use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_87121491: FileFormat = FileFormat {
    id: 87_121_491,
    source_type: SourceType::Wikidata,
    name: "Open Financial Exchange 1.6",
    extensions: &["ofx", "qfx"],
    media_types: &["application/x-ofx"],
    signatures: &[],
    related_formats: &[],
};
