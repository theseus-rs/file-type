use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111182163: FileFormat = FileFormat {
    id: 111_182_163,
    puid: "wikidata/111182163",
    name: "Dreamweaver Webpage Template",
    extensions: &["dwt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
