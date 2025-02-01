use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_117457063: FileFormat = FileFormat {
    id: 117_457_063,
    puid: "wikidata/117457063",
    name: "Microsoft Access Encrypted Database File 1.1",
    extensions: &["mda", "mdb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
