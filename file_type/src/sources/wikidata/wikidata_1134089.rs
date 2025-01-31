use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_1134089: FileFormat = FileFormat {
    id: 1_134_089,
    puid: "wikidata/1134089",
    name: "world file",
    extensions: &[
        "bilw", "blw", "bpw", "btw", "gfw", "jgw", "jpgw", "pgw", "rasterw", "sdw", "tfw", "tifw",
    ],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
