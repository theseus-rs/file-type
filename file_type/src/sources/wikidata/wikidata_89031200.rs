use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_89031200: FileFormat = FileFormat {
    id: 89_031_200,
    puid: "wikidata/89031200",
    name: "The Print Shop Project 2-5",
    extensions: &["psproj"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
