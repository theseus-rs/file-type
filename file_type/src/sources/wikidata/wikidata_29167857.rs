use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_29167857: FileFormat = FileFormat {
    id: 29_167_857,
    source_type: SourceType::Wikidata,
    name: "P-touch Editor Lite Label",
    extensions: &["lbt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
