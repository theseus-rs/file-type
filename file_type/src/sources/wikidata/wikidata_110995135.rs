use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_110995135: FileFormat = FileFormat {
    id: 110_995_135,
    puid: "wikidata/110995135",
    name: "Asymetrix 3D Scene",
    extensions: &["scn"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
