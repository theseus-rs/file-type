use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111355400: FileFormat = FileFormat {
    id: 111_355_400,
    puid: "wikidata/111355400",
    name: "Annotated speech file",
    extensions: &["vap"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
