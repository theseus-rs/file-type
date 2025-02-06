use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_130548774: FileFormat = FileFormat {
    id: 130_548_774,
    source_type: SourceType::Wikidata,
    name: "Qlik file format",
    extensions: &["qvs", "qvw"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
