use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_130548774: FileFormat = FileFormat {
    id: 130_548_774,
    source_type: SourceType::Wikidata,
    name: "Qlik file format",
    extensions: &["qvs", "qvw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
