use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_5310: FileFormat = FileFormat {
    id: 5_310,
    puid: "wikidata/5310",
    name: "LaTeX",
    extensions: &["tex"],
    media_types: &["application/x-latex"],
    internal_signatures: &[],
    related_formats: &[],
};
