use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_110458180: FileFormat = FileFormat {
    id: 110_458_180,
    source_type: SourceType::Wikidata,
    name: "Beam Software SIFF File",
    extensions: &["son", "vb"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
