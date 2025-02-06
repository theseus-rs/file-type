use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_110135637: FileFormat = FileFormat {
    id: 110_135_637,
    source_type: SourceType::Wikidata,
    name: "Serif PhotoPlus Image, version 5-X3",
    extensions: &["spp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
