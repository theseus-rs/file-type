use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_34745227: FileFormat = FileFormat {
    id: 34_745_227,
    source_type: SourceType::Wikidata,
    name: "Spline Font Database",
    extensions: &["sfd"],
    media_types: &["application/vnd.font-fontforge-sfd"],
    internal_signatures: &[],
    related_formats: &[],
};
