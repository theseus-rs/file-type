use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_123202980: FileFormat = FileFormat {
    id: 123_202_980,
    source_type: SourceType::Wikidata,
    name: "Microsoft DV-AVI Video File",
    extensions: &["dv-avi"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
