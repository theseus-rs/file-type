use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_21848765: FileFormat = FileFormat {
    id: 21_848_765,
    source_type: SourceType::Wikidata,
    name: "BioSemi Data Format",
    extensions: &["bdf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
