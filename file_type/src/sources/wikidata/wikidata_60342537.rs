use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_60342537: FileFormat = FileFormat {
    id: 60_342_537,
    puid: "wikidata/60342537",
    name: "SmartDraw format",
    extensions: &["sdr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
