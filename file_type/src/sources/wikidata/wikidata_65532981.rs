use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_65532981: FileFormat = FileFormat {
    id: 65_532_981,
    puid: "wikidata/65532981",
    name: "Cookbook file format",
    extensions: &["mc2"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
