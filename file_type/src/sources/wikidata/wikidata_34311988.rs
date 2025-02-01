use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_34311988: FileFormat = FileFormat {
    id: 34_311_988,
    puid: "wikidata/34311988",
    name: "Shen script",
    extensions: &["shen", "shen"],
    media_types: &["application/x-shen", "text/x-shen"],
    internal_signatures: &[],
    related_formats: &[],
};
