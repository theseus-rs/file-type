use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_86451664: FileFormat = FileFormat {
    id: 86_451_664,
    puid: "wikidata/86451664",
    name: "RFFlow Chart 3",
    extensions: &["flo"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
