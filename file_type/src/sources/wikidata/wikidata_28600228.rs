use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28600228: FileFormat = FileFormat {
    id: 28_600_228,
    source_type: SourceType::Wikidata,
    name: "APL workspace",
    extensions: &["apl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
