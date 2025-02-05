use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_130977039: FileFormat = FileFormat {
    id: 130_977_039,
    source_type: SourceType::Wikidata,
    name: "Singularity definition file",
    extensions: &["Singularity", "def"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
