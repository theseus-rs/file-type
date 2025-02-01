use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_119217744: FileFormat = FileFormat {
    id: 119_217_744,
    puid: "wikidata/119217744",
    name: "QuickBooks Accountant's Copy Work File",
    extensions: &["qba"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
