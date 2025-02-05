use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_1134089: FileFormat = FileFormat {
    id: 1_134_089,
    source_type: SourceType::Wikidata,
    name: "world file",
    extensions: &[
        "bilw", "blw", "bpw", "btw", "gfw", "jgw", "jpgw", "pgw", "rasterw", "sdw", "tfw", "tifw",
    ],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
