use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_127514871: FileFormat = FileFormat {
    id: 127_514_871,
    puid: "wikidata/127514871",
    name: "Text2tags file",
    extensions: &["t2t"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
