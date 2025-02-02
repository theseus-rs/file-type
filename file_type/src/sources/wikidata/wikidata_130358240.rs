use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_130358240: FileFormat = FileFormat {
    id: 130_358_240,
    source_type: SourceType::Wikidata,
    name: "Mscgen file format",
    extensions: &["msc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
