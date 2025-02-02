use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28205824: FileFormat = FileFormat {
    id: 28_205_824,
    source_type: SourceType::Wikidata,
    name: "CgBI",
    extensions: &["png"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
