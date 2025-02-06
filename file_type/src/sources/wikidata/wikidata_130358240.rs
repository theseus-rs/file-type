use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_130358240: FileFormat = FileFormat {
    id: 130_358_240,
    source_type: SourceType::Wikidata,
    name: "Mscgen file format",
    extensions: &["msc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
