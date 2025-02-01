use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111272661: FileFormat = FileFormat {
    id: 111_272_661,
    puid: "wikidata/111272661",
    name: "Ensoniq EPS family compacted disk image",
    extensions: &["eui"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
