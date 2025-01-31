use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_52005598: FileFormat = FileFormat {
    id: 52_005_598,
    puid: "wikidata/52005598",
    name: "AMI Draw Vector Image",
    extensions: &["sdw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
