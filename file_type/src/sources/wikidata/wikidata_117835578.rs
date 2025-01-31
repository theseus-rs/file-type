use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_117835578: FileFormat = FileFormat {
    id: 117_835_578,
    puid: "wikidata/117835578",
    name: "DataBeam file",
    extensions: &["dbx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
