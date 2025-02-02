use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27979404: FileFormat = FileFormat {
    id: 27_979_404,
    source_type: SourceType::Wikidata,
    name: "PICS",
    extensions: &["pcs"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
