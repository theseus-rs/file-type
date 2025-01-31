use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_110135637: FileFormat = FileFormat {
    id: 110_135_637,
    puid: "wikidata/110135637",
    name: "Serif PhotoPlus Image, version 5-X3",
    extensions: &["spp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
