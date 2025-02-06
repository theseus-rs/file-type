use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_21848765: FileFormat = FileFormat {
    id: 21_848_765,
    source_type: SourceType::Wikidata,
    name: "BioSemi Data Format",
    extensions: &["bdf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
