use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_121093863: FileFormat = FileFormat {
    id: 121_093_863,
    puid: "wikidata/121093863",
    name: "Punch! Home Suite PHD file",
    extensions: &["phd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
