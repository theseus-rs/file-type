use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_29167850: FileFormat = FileFormat {
    id: 29_167_850,
    source_type: SourceType::Wikidata,
    name: "P-touch Editor Label",
    extensions: &["lbx"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
