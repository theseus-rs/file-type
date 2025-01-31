use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111342726: FileFormat = FileFormat {
    id: 111_342_726,
    puid: "wikidata/111342726",
    name: "SPPack sound sample",
    extensions: &["sppack"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
