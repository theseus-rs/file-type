use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_29167850: FileFormat = FileFormat {
    id: 29_167_850,
    source_type: SourceType::Wikidata,
    name: "P-touch Editor Label",
    extensions: &["lbx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
