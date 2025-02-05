use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28207241: FileFormat = FileFormat {
    id: 28_207_241,
    source_type: SourceType::Wikidata,
    name: "SBIG CCDOPS image",
    extensions: &["sbig"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
