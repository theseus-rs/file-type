use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_70357595: FileFormat = FileFormat {
    id: 70_357_595,
    puid: "wikidata/70357595",
    name: "Jupyter notebook file",
    extensions: &["ipynb"],
    media_types: &["application/x-ipynb+json"],
    internal_signatures: &[],
    related_formats: &[],
};
