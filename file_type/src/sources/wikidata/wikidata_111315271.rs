use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111315271: FileFormat = FileFormat {
    id: 111_315_271,
    puid: "wikidata/111315271",
    name: "Gravis UltraSnd.ini bank setup",
    extensions: &["ini"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
