use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111418325: FileFormat = FileFormat {
    id: 111_418_325,
    puid: "wikidata/111418325",
    name: "Adobe Bridge Workspace File",
    extensions: &["workspace"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
