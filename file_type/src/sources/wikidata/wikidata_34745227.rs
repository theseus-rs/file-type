use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_34745227: FileFormat = FileFormat {
    id: 34_745_227,
    puid: "wikidata/34745227",
    name: "Spline Font Database",
    extensions: &["sfd"],
    media_types: &["application/vnd.font-fontforge-sfd"],
    internal_signatures: &[],
    related_formats: &[],
};
